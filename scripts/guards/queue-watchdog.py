#!/usr/bin/env python3
"""Merge-queue watchdog (A28 challenge, G183).

A merge_group job can hang: every step is completed but the job stays
`in_progress` (seen on #212 run 39620910334 and #223). The queue then waits
up to the 6 h job timeout. The working fix is GraphQL `dequeuePullRequest`;
the PR owner re-arms auto-merge afterwards.

For each entry of the main merge queue, this script reads the merge_group runs
of the entry's head commit and their jobs. A job is STUCK when:
  - its status is `in_progress`,
  - it started more than 10 minutes ago,
  - it has at least one step and every step is `completed`.
A long step that is still running (e.g. Runs "Re-run manifests") is live, not
stuck. For each entry with a stuck job, the PR is dequeued once and the PR,
run id, job name and time are written to the job summary.

It never re-arms auto-merge, never writes to the repo, never touches settings.

Usage:
  queue-watchdog.py run --repo OWNER/NAME [--dry-run]       (live, GITHUB_TOKEN)
  queue-watchdog.py run --snapshot FILE --now ISO [--dry-run]  (offline, tests)
Snapshot format: {"entries": [{"pr": N, "pr_id": "...", "head_sha": "...",
  "runs": [{"id": N, "jobs": [{"name", "status", "started_at",
  "steps": [{"name", "status"}]}]}]}]}
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import sys
import urllib.request

STUCK_AFTER = dt.timedelta(minutes=10)
API = "https://api.github.com"


def parse_time(value: str) -> dt.datetime:
    return dt.datetime.fromisoformat(value.replace("Z", "+00:00"))


def stuck_job(job: dict, now: dt.datetime) -> bool:
    if job.get("status") != "in_progress" or not job.get("started_at"):
        return False
    steps = job.get("steps") or []
    if not steps or any(s.get("status") != "completed" for s in steps):
        return False
    return now - parse_time(job["started_at"]) > STUCK_AFTER


def decide(snapshot: dict, now: dt.datetime) -> list[dict]:
    """One action per queue entry that has at least one stuck job."""
    actions = []
    for entry in snapshot.get("entries", []):
        for run in entry.get("runs", []):
            hit = next((j for j in run.get("jobs", []) if stuck_job(j, now)), None)
            if hit:
                actions.append({
                    "pr": entry["pr"], "pr_id": entry["pr_id"],
                    "run_id": run["id"], "job": hit["name"],
                    "started_at": hit["started_at"],
                })
                break
    return actions


def _request(url: str, token: str, body: dict | None = None) -> dict:
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, method="POST" if data else "GET")
    req.add_header("Authorization", f"Bearer {token}")
    req.add_header("Accept", "application/vnd.github+json")
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.load(resp)


def graphql(token: str, query: str, variables: dict) -> dict:
    out = _request(f"{API}/graphql", token, {"query": query, "variables": variables})
    if out.get("errors"):
        raise RuntimeError(json.dumps(out["errors"]))
    return out["data"]


QUEUE_QUERY = """query($owner: String!, $name: String!) {
  repository(owner: $owner, name: $name) {
    mergeQueue(branch: "main") {
      entries(first: 50) { nodes { headCommit { oid } pullRequest { number id } } }
    }
  }
}"""

DEQUEUE = """mutation($id: ID!) {
  dequeuePullRequest(input: {id: $id}) { mergeQueueEntry { id } }
}"""


def live_snapshot(repo: str, token: str) -> dict:
    owner, name = repo.split("/")
    data = graphql(token, QUEUE_QUERY, {"owner": owner, "name": name})
    queue = (data["repository"] or {}).get("mergeQueue") or {}
    entries = []
    for node in (queue.get("entries") or {}).get("nodes", []):
        sha = node["headCommit"]["oid"]
        runs_url = f"{API}/repos/{repo}/actions/runs?event=merge_group&head_sha={sha}"
        runs = []
        for run in _request(runs_url, token).get("workflow_runs", []):
            jobs_url = f"{API}/repos/{repo}/actions/runs/{run['id']}/jobs?filter=latest"
            jobs = _request(jobs_url, token).get("jobs", [])
            runs.append({"id": run["id"], "jobs": jobs})
        pr = node["pullRequest"]
        entries.append({"pr": pr["number"], "pr_id": pr["id"],
                        "head_sha": sha, "runs": runs})
    return {"entries": entries}


def summary(lines: list[str]) -> None:
    for line in lines:
        print(line)
    path = os.environ.get("GITHUB_STEP_SUMMARY")
    if path:
        with open(path, "a", encoding="utf-8") as fh:
            fh.write("\n".join(lines) + "\n")


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    run = sub.add_parser("run")
    run.add_argument("--repo")
    run.add_argument("--snapshot")
    run.add_argument("--now")
    run.add_argument("--dry-run", action="store_true")
    args = ap.parse_args(argv)

    if args.snapshot:
        with open(args.snapshot, encoding="utf-8") as fh:
            snap = json.load(fh)
        now = parse_time(args.now) if args.now else dt.datetime.now(dt.timezone.utc)
        token = None
    else:
        token = os.environ.get("GITHUB_TOKEN")
        if not args.repo or not token:
            print("queue-watchdog: --repo and GITHUB_TOKEN are required", file=sys.stderr)
            return 2
        snap = live_snapshot(args.repo, token)
        now = dt.datetime.now(dt.timezone.utc)

    stamp = now.strftime("%Y-%m-%dT%H:%M:%SZ")
    if not snap.get("entries"):
        summary([f"queue-watchdog {stamp}: merge queue empty, nothing to do."])
        return 0
    actions = decide(snap, now)
    if not actions:
        summary([f"queue-watchdog {stamp}: {len(snap['entries'])} queue entries, no stuck job."])
        return 0

    lines = [f"### queue-watchdog {stamp}", "",
             "| PR | run | job | job started | action |", "|---|---|---|---|---|"]
    failed = 0
    for act in actions:
        if args.dry_run or token is None:
            result = "DEQUEUE (dry run)"
        else:
            try:
                graphql(token, DEQUEUE, {"id": act["pr_id"]})
                result = "DEQUEUED"
            except (RuntimeError, OSError) as err:
                result = f"DEQUEUE FAILED: {err}"
                failed += 1
        lines.append(f"| #{act['pr']} | {act['run_id']} | {act['job']} | "
                     f"{act['started_at']} | {result} |")
    lines += ["", "Not re-armed: the PR owner re-enables auto-merge."]
    summary(lines)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

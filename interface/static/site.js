/* Astra — small dependency-free script.
   1. Text filter: <input data-filter="#target"> hides the [data-search] items of the target.
   2. Category filters: <select data-facet="key" data-facet-target="#target"> compares with data-key.
   3. "Hide resolved" box: <input type="checkbox" data-hide-resolved="#target"> (data-resolved="yes").
   4. Tooltips: <abbr data-term="id"> shows the glossary definition (JSON embedded in the page).
   Works when files are opened directly (file://), no server. */
(function () {
  "use strict";

  function norm(s) {
    return (s || "").toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "");
  }

  /* ---------- Filters ---------- */
  var containers = new Set();
  document.querySelectorAll("[data-filter]").forEach(function (el) { containers.add(el.dataset.filter); });
  document.querySelectorAll("[data-facet-target]").forEach(function (el) { containers.add(el.dataset.facetTarget); });
  document.querySelectorAll("[data-hide-resolved]").forEach(function (el) { containers.add(el.dataset.hideResolved); });

  function apply(sel) {
    var root = document.querySelector(sel);
    if (!root) return;
    var text = document.querySelector('[data-filter="' + sel + '"]');
    var words = norm(text ? text.value : "").split(/\s+/).filter(Boolean);
    var facets = Array.prototype.slice.call(document.querySelectorAll('[data-facet-target="' + sel + '"]'))
      .filter(function (f) { return f.value; });
    var hideBox = document.querySelector('[data-hide-resolved="' + sel + '"]');
    var hideResolved = hideBox && hideBox.checked;

    var items = root.querySelectorAll("[data-search]");
    var shown = 0;
    items.forEach(function (it) {
      var hay = it._hay || (it._hay = norm(it.dataset.search));
      var ok = words.every(function (w) { return hay.indexOf(w) !== -1; });
      if (ok) ok = facets.every(function (f) { return it.getAttribute("data-" + f.dataset.facet) === f.value; });
      if (ok && hideResolved) ok = it.dataset.resolved !== "yes";
      it.hidden = !ok;
      if (ok) shown++;
    });
    // Hide a group (section) whose items are all hidden.
    root.querySelectorAll("[data-group]").forEach(function (g) {
      var any = g.querySelector("[data-search]:not([hidden])");
      var hasItems = g.querySelector("[data-search]");
      g.hidden = Boolean(hasItems) && !any && (words.length > 0 || facets.length > 0);
    });
    var counter = document.querySelector('[data-count-for="' + sel + '"]');
    if (counter) counter.textContent = (words.length || facets.length || hideResolved) ? shown + " / " + items.length : items.length + " items";
  }

  containers.forEach(function (sel) {
    document.querySelectorAll('[data-filter="' + sel + '"], [data-facet-target="' + sel + '"], [data-hide-resolved="' + sel + '"]')
      .forEach(function (ctrl) {
        ctrl.addEventListener(ctrl.tagName === "INPUT" && ctrl.type === "search" ? "input" : "change", function () { apply(sel); });
      });
    apply(sel);
  });

  // "/" key: jump to the search box.
  document.addEventListener("keydown", function (e) {
    if (e.key === "/" && !/INPUT|SELECT|TEXTAREA/.test(document.activeElement.tagName)) {
      var f = document.querySelector("[data-filter]");
      if (f) { e.preventDefault(); f.focus(); }
    }
  });

  /* ---------- Glossary tooltips ---------- */
  var glossary = {};
  try {
    var raw = document.getElementById("glossary-data");
    var list = raw ? JSON.parse(raw.textContent) : [];
    (Array.isArray(list) ? list : Object.values(list)).forEach(function (t) { glossary[t.id] = t; });
  } catch (err) { /* no glossary: no tooltips */ }

  var root = document.querySelector('link[rel="stylesheet"]').getAttribute("href").replace(/static\/style\.css$/, "");
  var tip = null;

  function hide() { if (tip) { tip.remove(); tip = null; } }
  function show(el) {
    var t = glossary[el.dataset.term];
    if (!t) return;
    hide();
    tip = document.createElement("div");
    tip.className = "tooltip";
    tip.setAttribute("role", "tooltip");
    var title = document.createElement("strong");
    title.textContent = t.term;
    var body = document.createElement("span");
    body.textContent = t.definition + " ";
    var more = document.createElement("a");
    more.href = root + "glossary.html#" + t.id;
    more.textContent = "Glossary →";
    tip.append(title, body, more);
    document.body.appendChild(tip);
    var r = el.getBoundingClientRect();
    var left = Math.min(window.scrollX + r.left, window.scrollX + document.documentElement.clientWidth - tip.offsetWidth - 8);
    tip.style.left = Math.max(8, left) + "px";
    tip.style.top = (window.scrollY + r.bottom + 6) + "px";
  }

  document.querySelectorAll("abbr[data-term]").forEach(function (el) {
    var t = glossary[el.dataset.term];
    if (!t) return;
    el.tabIndex = 0;
    el.removeAttribute("title"); // avoid the browser double tooltip
    el.addEventListener("mouseenter", function () { show(el); });
    el.addEventListener("focus", function () { show(el); });
    el.addEventListener("click", function (e) { e.stopPropagation(); tip ? hide() : show(el); });
    el.addEventListener("blur", function () { setTimeout(function () { if (tip && !tip.matches(":hover")) hide(); }, 150); });
    el.addEventListener("mouseleave", function () { setTimeout(function () { if (tip && !tip.matches(":hover")) hide(); }, 250); });
  });
  document.addEventListener("click", function (e) { if (tip && !tip.contains(e.target)) hide(); });
  document.addEventListener("keydown", function (e) { if (e.key === "Escape") hide(); });
})();

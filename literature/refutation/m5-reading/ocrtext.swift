// A78 M5 — label-text OCR helper (rule 7a: OCR of tick label TEXT only).
//
// Usage: ./ocrtext <image.png> <x0> <y0> <x1> <y1>
// Prints one line per recognised text observation:
//   <text> | box minX,minY,maxX,maxY  (image pixel coords, origin top-left)
//
// This tool is only ever pointed at the narrow strip holding a y-axis tick
// LABELS or an x-axis label row. It is never used on bars, caps or plot areas.
import Foundation
import Vision
import AppKit

let args = CommandLine.arguments
guard args.count == 6 else {
    FileHandle.standardError.write("usage: ocrtext <img> <x0> <y0> <x1> <y1>\n".data(using: .utf8)!)
    exit(2)
}
let path = args[1]
let x0 = Int(args[2])!, y0 = Int(args[3])!, x1 = Int(args[4])!, y1 = Int(args[5])!

guard let src = NSImage(contentsOfFile: path),
      let tiff = src.tiffRepresentation,
      let rep = NSBitmapImageRep(data: tiff) else {
    FileHandle.standardError.write("cannot read image: \(path)\n".data(using: .utf8)!)
    exit(2)
}
let W = rep.pixelsWide, H = rep.pixelsHigh
let cx0 = max(0, min(x0, x1)), cx1 = min(W, max(x0, x1))
let cy0 = max(0, min(y0, y1)), cy1 = min(H, max(y0, y1))
let w = cx1 - cx0, h = cy1 - cy0
if w <= 0 || h <= 0 { FileHandle.standardError.write("empty crop\n".data(using: .utf8)!); exit(2) }

// Crop into a new CGImage.
guard let cg = rep.cgImage else { exit(2) }
guard let crop = cg.cropping(to: CGRect(x: cx0, y: cy0, width: w, height: h)) else {
    FileHandle.standardError.write("crop failed\n".data(using: .utf8)!)
    exit(2)
}

// Upscale 4x for small tick labels, which Vision reads far more reliably.
let scale = 4
let bw = w * scale, bh = h * scale
let cs = CGColorSpaceCreateDeviceRGB()
guard let ctx = CGContext(data: nil, width: bw, height: bh, bitsPerComponent: 8,
                          bytesPerRow: 0, space: cs,
                          bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue) else { exit(2) }
ctx.interpolationQuality = .high
ctx.draw(crop, in: CGRect(x: 0, y: 0, width: bw, height: bh))
guard let big = ctx.makeImage() else { exit(2) }

let request = VNRecognizeTextRequest()
request.recognitionLevel = .accurate
request.usesLanguageCorrection = false
request.recognitionLanguages = ["en-US"]
// Digits, decimal points, minus signs and a few unit letters.
request.customWords = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                       "0.0", "0.5", "1.0", "1.5", "2.0", "2.5", "3.0"]

let handler = VNImageRequestHandler(cgImage: big, options: [:])
do { try handler.perform([request]) } catch {
    FileHandle.standardError.write("vision error: \(error)\n".data(using: .utf8)!)
    exit(3)
}

// Vision boxes are normalised, origin bottom-left of the crop.
for obs in (request.results ?? []) {
    guard let top = obs.topCandidates(1).first else { continue }
    let b = obs.boundingBox
    let bx0 = Double(cx0) + Double(b.minX) * Double(w)
    let bx1 = Double(cx0) + Double(b.maxX) * Double(w)
    // flip y into image coords (origin top-left)
    let by0 = Double(cy0) + (1.0 - Double(b.maxY)) * Double(h)
    let by1 = Double(cy0) + (1.0 - Double(b.minY)) * Double(h)
    print(String(format: "%@ | %.1f,%.1f,%.1f,%.1f | conf %.2f",
                 top.string, bx0, by0, bx1, by1, top.confidence))
}

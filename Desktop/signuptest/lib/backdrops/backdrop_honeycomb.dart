import 'dart:math' as math;
import 'dart:ui' as ui;
import 'package:flutter/material.dart';

class BackdropHoneycomb extends StatelessWidget {
  final Animation<double> animation;
  const BackdropHoneycomb({super.key, required this.animation});

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: animation,
      builder: (_, __) =>
          CustomPaint(painter: _HoneycombPainter(t: animation.value)),
    );
  }
}

class _HoneycombPainter extends CustomPainter {
  final double t;
  _HoneycombPainter({required this.t});

  @override
  void paint(Canvas canvas, Size size) {
    final base = Paint()
      ..shader = ui.Gradient.linear(
        const Offset(0, 0),
        Offset(0, size.height),
        const [Color(0xFFB80F0A), Color(0xFF930D09)],
      );
    canvas.drawRect(Offset.zero & size, base);

    final vignette = Paint()
      ..shader = ui.Gradient.radial(
        Offset(size.width * .55, size.height * .40),
        size.shortestSide * 0.95,
        const [Color(0x00000000), Color(0x30000000)],
      );
    canvas.drawRect(Offset.zero & size, vignette);

    final hexR = math.max(16.0, size.shortestSide * 0.018);
    final h = hexR * math.sqrt(3);
    final colCount = (size.width / (1.5 * hexR)).ceil() + 2;
    final rowCount = (size.height / h).ceil() + 2;

    final driftX = 8 * math.sin(t * 2 * math.pi);
    final driftY = 6 * math.cos(t * 2 * math.pi);

    final stroke = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1
      ..color = const Color(0x28FFFFFF);

    Path hexPath(double cx, double cy) {
      final p = Path();
      for (int k = 0; k < 6; k++) {
        final ang = math.pi / 3 * k + math.pi / 6;
        final x = cx + hexR * math.cos(ang);
        final y = cy + hexR * math.sin(ang);
        if (k == 0) {
          p.moveTo(x, y);
        } else {
          p.lineTo(x, y);
        }
      }
      p.close();
      return p;
    }

    for (int r = 0; r < rowCount; r++) {
      for (int c = 0; c < colCount; c++) {
        final cx = (1.5 * hexR) * c + driftX;
        final cy =
            h * r + ((c % 2 == 0) ? 0 : h / 2) + driftY;
        final p = hexPath(cx, cy);
        canvas.drawPath(p, stroke);
      }
    }

    final sweep = Paint()
      ..shader = ui.Gradient.linear(
        Offset(-size.width + t * size.width * 2, 0),
        Offset(t * size.width * 2, size.height),
        const [
          Color(0x12FFFFFF),
          Color(0x00FFFFFF),
          Color(0x18FFFFFF),
        ],
        const [0.15, 0.50, 0.85],
      )
      ..blendMode = BlendMode.softLight;
    canvas.drawRect(Offset.zero & size, sweep);
  }

  @override
  bool shouldRepaint(_HoneycombPainter oldDelegate) => oldDelegate.t != t;
}

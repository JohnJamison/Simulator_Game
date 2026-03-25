import 'dart:math' as math;
import 'dart:ui' as ui;
import 'package:flutter/material.dart';

class BackdropGrid extends StatelessWidget {
  final Animation<double> animation;
  const BackdropGrid({super.key, required this.animation});

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: animation,
      builder: (_, __) => CustomPaint(
        painter: _GridPainter(t: animation.value),
      ),
    );
  }
}

class _GridPainter extends CustomPainter {
  final double t;
  _GridPainter({required this.t});

  @override
  void paint(Canvas canvas, Size size) {
    final base = Paint()
      ..shader = ui.Gradient.linear(
        const Offset(0, 0),
        Offset(0, size.height),
        const [Color(0xFFB80F0A), Color(0xFFA20E09)],
      );
    canvas.drawRect(Offset.zero & size, base);

    final vignette = Paint()
      ..shader = ui.Gradient.radial(
        Offset(size.width * .5, size.height * .4),
        size.shortestSide * .9,
        const [Color(0x00000000), Color(0x22000000)],
      );
    canvas.drawRect(Offset.zero & size, vignette);

    final spacing = 64.0;
    final dx = spacing * (.5 * math.sin(t * math.pi * 2));
    final dy = spacing * (.5 * math.cos(t * math.pi * 2));
    final gridPaint = Paint()
      ..color = const Color(0x33FFFFFF)
      ..strokeWidth = 1;

    for (double x = -spacing; x <= size.width + spacing; x += spacing) {
      canvas.drawLine(
        Offset(x + dx, 0),
        Offset(x + dx, size.height),
        gridPaint,
      );
    }
    for (double y = -spacing; y <= size.height + spacing; y += spacing) {
      canvas.drawLine(
        Offset(0, y + dy),
        Offset(size.width, y + dy),
        gridPaint,
      );
    }

    final nodePaint = Paint()..color = const Color(0x22FFFFFF);
    for (double x = -spacing; x <= size.width + spacing; x += spacing) {
      for (double y = -spacing; y <= size.height + spacing; y += spacing) {
        final r = 2.0 +
            1.0 * (0.5 + 0.5 * math.sin((x + y) * .03 + t * 6.28));
        canvas.drawCircle(Offset(x + dx, y + dy), r, nodePaint);
      }
    }

    final shift = size.width * (.25 * math.sin(t * math.pi));
    final sheen = Paint()
      ..shader = ui.Gradient.linear(
        Offset(-size.width + shift, -size.height),
        Offset(size.width + shift, size.height),
        const [
          Color(0x18FFFFFF),
          Color(0x00FFFFFF),
          Color(0x10FFFFFF),
        ],
        const [0.25, 0.52, 0.78],
      )
      ..blendMode = BlendMode.softLight;
    canvas.drawRect(Offset.zero & size, sheen);
  }

  @override
  bool shouldRepaint(_GridPainter oldDelegate) => oldDelegate.t != t;
}

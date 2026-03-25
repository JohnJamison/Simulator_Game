import 'package:flutter/material.dart';

class Dots extends StatelessWidget {
  final int count;
  final int index;
  final Color active;
  final Color inactive;

  const Dots({
    super.key,
    required this.count,
    required this.index,
    required this.active,
    required this.inactive,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: List.generate(count, (i) {
        final on = i == index;
        return AnimatedContainer(
          duration: const Duration(milliseconds: 160),
          margin: const EdgeInsets.symmetric(horizontal: 4),
          height: 8,
          width: on ? 24 : 8,
          decoration: BoxDecoration(
            color: on ? active : inactive,
            borderRadius: BorderRadius.circular(999),
          ),
        );
      }),
    );
  }
}

import 'package:flutter/material.dart';
import '../modes/timed_mode.dart'; // for SnapSlider & SnapStyles

class StyleLab extends StatelessWidget {
  const StyleLab({super.key});

  @override
  Widget build(BuildContext context) {
    final labels = const ['Math','Reading','Writing','Mixed'];
    return Scaffold(
      appBar: AppBar(title: const Text('Style Lab')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          _card(context, 'Minimal (thin, modern)', SnapStyles.minimal(context), labels),
          _card(context, 'Filled (Material-ish)', SnapStyles.filled(context), labels),
          _card(context, 'Glass (brighter, no blur)', SnapStyles.glass(context), labels),
          _underlineCard(context, 'Underline (tab-like)', labels),
        ],
      ),
    );
  }

  Widget _card(BuildContext c, String title, SnapStyle style, List<String> labels) {
    int idx = 1;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text(title, style: Theme.of(c).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w800)),
          const SizedBox(height: 12),
          StatefulBuilder(builder: (context, set) {
            return SnapSlider(
              labels: labels,
              initialIndex: idx,
              onChanged: (i) => set(() => idx = i),
              style: style,
            );
          }),
        ]),
      ),
    );
  }

  Widget _underlineCard(BuildContext c, String title, List<String> labels) {
    int idx = 2;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text(title, style: Theme.of(c).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w800)),
          const SizedBox(height: 12),
          StatefulBuilder(builder: (context, set) {
            return SnapSlider(
              labels: labels,
              initialIndex: idx,
              onChanged: (i) => set(() => idx = i),
              style: SnapStyles.underline(c),
            );
          }),
        ]),
      ),
    );
  }
}

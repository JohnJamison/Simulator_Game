import 'package:flutter/material.dart';

class StatItem {
  final IconData icon;
  final String label;
  final String value;
  const StatItem({required this.icon, required this.label, required this.value});
}

class StatsGrid extends StatelessWidget {
  final List<StatItem> items;
  const StatsGrid({super.key, required this.items});

  @override
  Widget build(BuildContext context) {
    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      itemCount: items.length,
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 2, // 2x2 grid
        crossAxisSpacing: 12,
        mainAxisSpacing: 12,
        childAspectRatio: 1.9,
      ),
      itemBuilder: (_, i) {
        final it = items[i];
        return Card(
          elevation: 4,
          child: Padding(
            padding: const EdgeInsets.all(14),
            child: Row(
              children: [
                CircleAvatar(backgroundColor: Colors.black12, child: Icon(it.icon)),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(it.label, style: const TextStyle(color: Colors.black54)),
                      const SizedBox(height: 4),
                      Text(it.value, style: const TextStyle(fontWeight: FontWeight.w900, fontSize: 18)),
                    ],
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

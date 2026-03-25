import 'package:flutter/material.dart';

class RecentsRow extends StatelessWidget {
  final List<String> items;
  final void Function(String label) onTap;

  const RecentsRow({
    super.key,
    required this.items,
    required this.onTap,
  });

  // Map each mode to a relevant icon
  IconData _iconForMode(String mode) {
    switch (mode) {
      case 'Timed Mode':
        return Icons.timer_outlined;
      case 'Practice Sets':
        return Icons.list_alt_outlined;
      case 'Adaptive Drill':
        return Icons.auto_graph_outlined;
      default:
        return Icons.circle_outlined;
    }
  }


//================================= Build =================================
  @override
  Widget build(BuildContext context) {

    return Column(

      crossAxisAlignment: CrossAxisAlignment.start,
      children: [

        //------------ Recents Title -------------------------------------
        const Text(

          'Recents',
          style: TextStyle(

            fontSize: 30,
            fontWeight: FontWeight.bold,
            color: Colors.white,
            
          ),
        ),

        const SizedBox(height: 20),

        // ------------ Recent Clickables --------------------------------
        Column(

          children: items.map((label) {

            return Padding(
              
              padding: const EdgeInsets.symmetric(vertical: 8),
              child: InkWell(

                borderRadius: BorderRadius.circular(8),
                onTap: () => onTap(label),
                child: Row(

                  children: [

                    Icon(

                      _iconForMode(label),
                      color: Colors.white,
                      size: 40,
                    ),

                    const SizedBox(width: 12),

                    Text(

                      label,
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 16,
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                  ],
                ),
              ),
            );
          }).toList(),
        ),
      ],
    );
  }
}

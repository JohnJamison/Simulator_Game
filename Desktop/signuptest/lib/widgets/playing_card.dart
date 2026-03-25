import 'package:flutter/material.dart';
import '../models/mode_item.dart';

class PlayingCard extends StatelessWidget {
  final ModeItem item;
  final bool highlighted;
  final VoidCallback onStart;

  const PlayingCard({
    super.key,
    required this.item,
    required this.highlighted,
    required this.onStart,
  });

  @override
  Widget build(BuildContext context) {

    return ConstrainedBox(

      constraints: const BoxConstraints(maxHeight: 600),

      child: AspectRatio(

        //Card Size
        aspectRatio: 2.6 / 4,
        child: LayoutBuilder(builder: (context, c) {

          final ovalWidth = c.maxWidth * .70;
          return Stack(

            clipBehavior: Clip.none,
            children: [

              Positioned(

                left: (c.maxWidth - ovalWidth) / 2,
                right: (c.maxWidth - ovalWidth) / 2,
                bottom: 28,
                child: IgnorePointer(

                  child: AnimatedContainer(

                    duration: const Duration(milliseconds: 160),
                    height: 22,
                    decoration: BoxDecoration(

                      borderRadius: BorderRadius.circular(999),
                      color:
                          Colors.black.withOpacity(highlighted ? 0.22 : 0.14),
                      boxShadow: [
                        BoxShadow(
                          color:
                              Colors.black.withOpacity(highlighted ? .32 : .22),
                          blurRadius: highlighted ? 44 : 30,
                          spreadRadius: highlighted ? 12 : 8,
                        ),
                      ],
                    ),
                  ),
                ),
              ),
              Positioned.fill(
                top: 54,
                child: AnimatedContainer(
                  duration: const Duration(milliseconds: 160),
                  decoration: BoxDecoration(
                    color: Colors.white,
                    borderRadius: BorderRadius.circular(24),
                    border: Border.all(color: const Color(0xFFE9E2E1)),
                    boxShadow: [
                      BoxShadow(
                        color:
                            Colors.black.withOpacity(highlighted ? .30 : .18),
                        blurRadius: highlighted ? 46 : 24,
                        offset: const Offset(0, 22),
                      ),
                      if (highlighted)
                        BoxShadow(
                          color: const Color(0xFFB80F0A).withOpacity(.18),
                          blurRadius: 28,
                          offset: const Offset(0, 10),
                        ),
                    ],
                  ),
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(18, 62, 18, 16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        if (item.live)
                          Align(
                            alignment: Alignment.topRight,
                            child: Container(
                              padding: const EdgeInsets.symmetric(
                                  horizontal: 10, vertical: 6),
                              decoration: BoxDecoration(
                                color: const Color(0x14B80F0A),
                                borderRadius: BorderRadius.circular(999),
                                border: Border.all(
                                    color: const Color(0x40B80F0A)),
                              ),
                              child: const Text(
                                'LIVE',
                                style: TextStyle(
                                  color: Color(0xFFB80F0A),
                                  fontWeight: FontWeight.w900,
                                  letterSpacing: .3,
                                ),
                              ),
                            ),
                          ),
                        const SizedBox(height: 6),
                        Text(
                          item.title,
                          textAlign: TextAlign.center,
                          style: Theme.of(context)
                              .textTheme
                              .titleLarge
                              ?.copyWith(
                                  fontWeight: FontWeight.w900,
                                  color: const Color(0xFF242323)),
                        ),
                        const SizedBox(height: 10),
                        Text(
                          item.description,
                          textAlign: TextAlign.center,
                          style: Theme.of(context)
                              .textTheme
                              .bodyMedium
                              ?.copyWith(color: const Color(0xFF6E6A69)),
                        ),
                        const Spacer(),
                        Wrap(
                          spacing: 12,
                          alignment: WrapAlignment.center,
                          children: [
                            FilledButton.icon(
                              onPressed: onStart,
                              icon: Icon(item.live
                                  ? Icons.play_arrow
                                  : Icons.lock_clock),
                              label: Text(
                                  item.live ? 'Start' : 'Coming soon'),
                            ),
                            OutlinedButton.icon(
                              onPressed: () {},
                              icon: const Icon(Icons.info_outline),
                              label: const Text('Details'),
                              style: OutlinedButton.styleFrom(
                                foregroundColor:
                                    const Color(0xFF6E6A69),
                                side: const BorderSide(
                                    color: Color(0xFFE6E0DE)),
                                shape: const StadiumBorder(),
                              ),
                            ),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
              ),
              Positioned(
                left: 0,
                right: 0,
                top: -10,
                child: Center(
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 160),
                    width: highlighted ? 100 : 94,
                    height: highlighted ? 100 : 94,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      gradient: const LinearGradient(
                        colors: [Color(0xFFFFE1DF), Color(0xFFFBEAE8)],
                        begin: Alignment.topLeft,
                        end: Alignment.bottomRight,
                      ),
                      border: Border.all(color: Colors.white, width: 6),
                      boxShadow: [
                        BoxShadow(
                          color:
                              Colors.black.withOpacity(highlighted ? .28 : .18),
                          blurRadius: highlighted ? 22 : 16,
                          offset: const Offset(0, 10),
                        ),
                      ],
                    ),
                    child: CircleAvatar(
                      backgroundColor: const Color(0xFFF7E4E3),
                      child: Icon(
                        item.icon,
                        size: 32,
                        color: const Color(0xFFB80F0A),
                      ),
                    ),
                  ),
                ),
              ),
            ],
          );
        }),
      ),
    );
  }
}

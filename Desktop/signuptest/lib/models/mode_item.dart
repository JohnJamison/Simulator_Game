//________________________Mode Item___________________________
// 
//
//============================================================


import 'package:flutter/material.dart';

class ModeItem {
  final String id;
  final IconData icon;
  final String title;
  final String description;
  final bool live;

  const ModeItem({
    required this.id,
    required this.icon,
    required this.title,
    required this.description,
    this.live = false,
  });
}

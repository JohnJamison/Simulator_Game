import 'package:flutter/material.dart';

class Announcement {
  final IconData icon;
  final String title;
  final String? subtitle;
  final Color tint;
  const Announcement(this.icon, this.title, {this.subtitle, this.tint = const Color(0xFFB80F0A)});
}

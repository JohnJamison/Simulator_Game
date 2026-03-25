// lib/models/user_profile.dart

import '../models/user_events.dart';

class UserProfile {
  String id;           // some unique ID (you can use a UUID or whatever later)
  String username;     // must be unique
  String displayName;  // what they show to other users (can repeat)
  String email;        // login email


  String profilePhoto; // asset path or URL

  String country;
  String state;
  String city;

  List<String> friends;   // list of user IDs this user is friends with

  /// Time-series events: everything about their performance lives here.
  List<UserEvent> events;

  UserProfile({
    required this.id,
    required this.username,
    required this.displayName,
    required this.email,
    required this.profilePhoto,
    required this.country,
    required this.state,
    required this.city,
    required this.friends,
    required this.events,

  });

  // Convenience getter for total number of events (optional)
  int get totalEvents => events.length;

  // JSON for persistence (later when you hook up storage)
  Map<String, dynamic> toJson() => {
        'id': id,
        'username': username,
        'displayName': displayName,
        'email': email,
        'profilePhoto': profilePhoto,
        'country': country,
        'state': state,
        'city': city,
        'friends': friends,
        'events': events.map((e) => e.toJson()).toList(),
      };

  factory UserProfile.fromJson(Map<String, dynamic> json) => UserProfile(
        id: json['id'] as String,
        username: json['username'] as String,
        displayName: json['displayName'] as String,
        email: json['email'] as String,
        profilePhoto: json['profilePhoto'] as String,
        country: json['country'] as String,
        state: json['state'] as String,
        city: json['city'] as String,
        friends: List<String>.from(json['friends'] ?? []),
        events: (json['events'] as List<dynamic>? ?? [])
            .map((e) => UserEvent.fromJson(e as Map<String, dynamic>))
            .toList(),
      );
}
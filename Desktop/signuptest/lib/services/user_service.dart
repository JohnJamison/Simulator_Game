// lib/services/user_service.dart

import 'dart:math';
import 'package:uuid/uuid.dart';
import '../models/user_profile.dart';

class UserService {
  static final UserService instance = UserService._internal();
  UserService._internal();

  final _uuid = const Uuid();

  // Simulated "database"
  final Map<String, UserProfile> _users = {};

  UserProfile? currentUser;

  // -------- Create a new user --------
  UserProfile createUser({
    required String username,
    required String displayName,
    required String email,
  }) {
    if (!_isUsernameAvailable(username)) {
      throw Exception("Username already taken");
    }

    final user = UserProfile(
      id: _uuid.v4(),
      username: username,
      displayName: displayName,
      email: email,
      profilePhoto: "assets/profile/default_pfp.png",
      country: "",
      state: "",
      city: "",
      friends: [],
      events: [], // Added events since not there previously
    );

    _users[user.id] = user;
    currentUser = user;
    return user;
  }

  // -------- Username availability check --------
  bool _isUsernameAvailable(String username) {
    return !_users.values.any((u) => u.username.toLowerCase() == username.toLowerCase());
  }

  // Public API:
  bool usernameAvailable(String username) => _isUsernameAvailable(username);

  // -------- Update user --------
  void updateUser(UserProfile updated) {
    _users[updated.id] = updated;
    currentUser = updated;
  }

  // -------- Get a user by id --------
  UserProfile? getUser(String id) => _users[id];
}
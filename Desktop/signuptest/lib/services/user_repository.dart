import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart';

class UserRepository {
  static final _firestore = FirebaseFirestore.instance;

  /// Firestore doc reference for the current user
  static DocumentReference<Map<String, dynamic>> get _userDoc {
    final uid = FirebaseAuth.instance.currentUser?.uid;
    if (uid == null) throw Exception('No logged-in user');
    return _firestore.collection('users').doc(uid);
  }

  /// Create Firestore profile after sign-up
  static Future<void> createUserProfile({
    required String email,
    required String username,
  }) async {
    final uid = FirebaseAuth.instance.currentUser!.uid;

    await _firestore.collection('users').doc(uid).set({
      'email': email,
      'username': username,
      'name': username,
      'accuracy': 0,
      'avgTime': 0,
      'scoreEstimate': 1200,
      'tags': [],
      'friends': [],
      'createdAt': FieldValue.serverTimestamp(),
    });
  }

  /// Stream the user's profile in real-time
  static Stream<DocumentSnapshot<Map<String, dynamic>>> profileStream() {
    return _userDoc.snapshots();
  }

  /// Get the profile once
  static Future<DocumentSnapshot<Map<String, dynamic>>> getProfile() {
    return _userDoc.get();
  }

  /// Update any field(s)
  static Future<void> updateProfile(Map<String, dynamic> data) {
    return _userDoc.update(data);
  }
}

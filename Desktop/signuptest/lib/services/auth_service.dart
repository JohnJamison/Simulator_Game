// lib/services/auth_service.dart
import 'package:firebase_auth/firebase_auth.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:google_sign_in/google_sign_in.dart';
import 'package:uuid/uuid.dart';

import '../models/user_profile.dart';
import '../models/user_events.dart';

class AuthService {
  static final FirebaseAuth _auth = FirebaseAuth.instance;
  static final FirebaseFirestore _db = FirebaseFirestore.instance;
  static final GoogleSignIn _googleSignIn = GoogleSignIn(scopes: ['email']);
  /// Stream of auth state changes (null = signed out)
  static Stream<User?> get authStateChanges => _auth.authStateChanges();


  /// Get currently signed-in Firebase user
  static User? get currentUser => _auth.currentUser;

  /// Fetch UserProfile from Firestore
  static Future<UserProfile?> getUserProfile() async {
    final user = currentUser;
    if (user == null) return null;

    final doc = await _db.collection('users').doc(user.uid).get();
    if (!doc.exists) return null;

    return UserProfile.fromJson(doc.data() as Map<String, dynamic>);
  }

  /// Email/Password login
  static Future<UserProfile?> loginWithEmail(String email, String password) async {
    UserCredential cred = await _auth.signInWithEmailAndPassword(email: email, password: password);
    return getUserProfile();
  }

  /// Google Sign-In
  static Future<UserProfile?> loginWithGoogle() async {
    final googleUser = await _googleSignIn.signIn();
    if (googleUser == null) return null;

    final googleAuth = await googleUser.authentication;
    final credential = GoogleAuthProvider.credential(
      accessToken: googleAuth.accessToken,
      idToken: googleAuth.idToken,
    );

    UserCredential cred = await _auth.signInWithCredential(credential);

    final firebaseUid = cred.user!.uid;

    // Create profile if new user
    final doc = await _db.collection('users').doc(firebaseUid).get();

    if (!doc.exists) {
      final profile = UserProfile(
        id: firebaseUid,
        username: googleUser.email.split('@')[0],
        displayName: googleUser.displayName ?? '',
        email: googleUser.email,
        profilePhoto: googleUser.photoUrl ?? '',
        country: '',
        state: '',
        city: '',
        friends: [],
        events: [],
      );

      await _db.collection('users').doc(firebaseUid).set(profile.toJson());
      return profile;
    } else {
      return UserProfile.fromJson(doc.data() as Map<String, dynamic>);
    }
  }

  /// Sign out
  static Future<void> signOut() async {
    await _auth.signOut();
    await _googleSignIn.signOut();
  }
}

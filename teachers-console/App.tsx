import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View } from 'react-native';
import { Routes, Route } from "react-router-dom";
import { LoginPage } from "./pages/Login";
import { HomePage } from "./pages/Home";
import "./styles.css";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/login" element={<LoginPage />} />
    </Routes>
  );
}
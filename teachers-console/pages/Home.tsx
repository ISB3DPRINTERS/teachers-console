import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View } from 'react-native';
import { useState, useEffect } from 'react'
import { createClient } from '@supabase/supabase-js'
import { Auth } from '@supabase/auth-ui-react'
import { ThemeSupa} from '@supabase/auth-ui-shared'
import supabase from '../api/supabase'
import React from 'react';

export default async function loggedin() {
    const logoutfunc = async () => {
        return await supabase.auth.signOut()
    }
    return (
        <div>
            <h1>Administrator Portal</h1>
            <p>This page has admin features</p>
            <div>
                <p>current token </p>
            </div>
            <button onClick={() => {
                fs.writeFileSync("test.txt", "Hello, world!")
            }}>Force Regenerate Keys (use this when students are misusing app)</button>
            <button onClick={await logoutfunc()}>Logout</button>
        </div>
    )
}
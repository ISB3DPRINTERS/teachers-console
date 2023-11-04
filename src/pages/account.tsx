import { FC } from "react";
import * as React from 'react'
import getinfo from "../api/getinfo"
import { useState } from "react";

export const Account: FC<{ token: string }> = () => {
    const [loading, setLoading] = useState(false);
    const [initials, setInitials] = useState("");
    const [grade, setGrade] = useState("");

    const handleLogin = async (event: Event) => {
        event.preventDefault();

        setLoading(true);

        const res = await fetch("/api/resetpass", {
            method: "POST",

            headers: {
                "Content-Type": "application/json",
            },
            
            body: JSON.stringify({
                initials,
                grade,
            }),
        });

        const token = await res.text();

        localStorage.setItem("auth-token", token);

        setLoading(false);
    };

    return (
        <div>
            <h1>Teachers' Console</h1>
            <br></br>
            <strong>WARNING: Never give students access to this page</strong>
            <h3>Info Table</h3>
            <table>
                <tr>
                    <th>Grade</th>
                    <th>Daily PassKey</th>
                    <th>$ used (approx)</th>
                </tr>
                <tr>
                    <td>6th</td>
                    <td>{getinfo.getkey(6)}</td>
                    <td>{getinfo.getcost(6)}</td>
                </tr>
                <tr>
                    <td>7th</td>
                    <td>{getinfo.getkey(7)}</td>
                    <td>{getinfo.getcost(7)}</td>
                </tr>
                <tr>
                    <td>8th</td>
                    <td>{getinfo.getkey(8)}</td>
                    <td>{getinfo.getcost(8)}</td>
                </tr>
                <tr>
                    <td>9th</td>
                    <td>{getinfo.getkey(9)}</td>
                    <td>{getinfo.getcost(9)}</td>
                </tr>
                <tr>
                    <td>10th</td>
                    <td>{getinfo.getkey(10)}</td>
                    <td>{getinfo.getcost(10)}</td>
                </tr>
                <tr>
                    <td>11th</td>
                    <td>{getinfo.getkey(11)}</td>
                    <td>{getinfo.getcost(11)}</td>
                </tr>
                <tr>
                    <td>12th</td>
                    <td>{getinfo.getkey(12)}</td>
                    <td>{getinfo.getcost(12)}</td>
                </tr>
            </table>
            <br></br>
            <br></br>
            <h2>Reset PassKeys</h2>
            <div className="auth-form-container">
                <form className="form-widget" onSubmit={handleLogin as any}>
                    <div className="form-group">
                        <input
                            className="inputField"
                            type="text"
                            placeholder="Your Initials"
                            value={initials}
                            required={true}
                            onChange={(e) => setInitials(e.target.value)}
                        />
                        <input
                            className="inputField"
                            type="grade"
                            placeholder="Grade: Only # or ALL"
                            value={grade}
                            required={true}
                            onChange={(e) => setGrade(e.target.value)}
                        />
                    </div>

                        <button className="submit-button" disabled={loading}>
                            {loading ? (
                                <span>Loading</span>
                            ) : (
                                <span>Sign In</span>
                            )}
                        </button>
                </form>
            </div>
        </div>
    );
}
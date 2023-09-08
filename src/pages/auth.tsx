import "./auth.scss";
import { useState } from "react";

export default function Auth() {
    const [loading, setLoading] = useState(false);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const handleLogin = async (event: Event) => {
        event.preventDefault();

        setLoading(true);
        setLoading(false);
    };

    return (
        <div className="auth-form-container">
                <h1 className="header">Teacher Console</h1>
                
                <p className="description">
                    Sign in with your username and password below
                </p>

                <form className="form-widget" onSubmit={handleLogin as any}>
                    <div className="form-group">
                        <input
                            className="inputField"
                            type="text"
                            placeholder="Your username"
                            value={username}
                            required={true}
                            onChange={(e) => setUsername(e.target.value)}
                        />
                        <input
                            className="inputField"
                            type="password"
                            placeholder="Your password"
                            value={password}
                            required={true}
                            onChange={(e) => setPassword(e.target.value)}
                        />
                    </div>

                    <div>
                        <button className="submit-button" disabled={loading}>
                            {loading ? (
                                <span>Loading</span>
                            ) : (
                                <span>Sign In</span>
                            )}
                        </button>
                    </div>
                </form>
            </div>
    );
}

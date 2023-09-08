import { FC, useState } from "react";

export const Account: FC<{ session: any }> = () => {
    const [loading, setLoading] = useState(false);
    const [email, setEmail] = useState("");

    const handleLogin = async (event: Event) => {
        event.preventDefault();

        setLoading(true);
        setLoading(false);
    };

    return (
        <div className="row flex flex-center">
            <div className="col-6 form-widget">
                <h1 className="header">Supabase + React</h1>
                <p className="description">
                    Sign in via magic link with your email below
                </p>
                <form className="form-widget" onSubmit={handleLogin as any}>
                    <div>
                        <input
                            className="inputField"
                            type="email"
                            placeholder="Your email"
                            value={email}
                            required={true}
                            onChange={(e) => setEmail(e.target.value)}
                        />
                    </div>
                    <div>
                        <button className={"button block"} disabled={loading}>
                            {loading ? (
                                <span>Loading</span>
                            ) : (
                                <span>Send magic link</span>
                            )}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
}

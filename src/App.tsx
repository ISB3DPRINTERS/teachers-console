import { useState } from "react";
import Auth from "./pages/auth";
import { Account } from "./pages/account";

export const App = () => {
    const [session] = useState(null as any);

    return (
        <div className="container" style={{ padding: "50px 0 100px 0" }}>
            {!session ? (
                <Auth />
            ) : (
                <Account key={session.user.id} session={session} />
            )}
        </div>
    );
}

export default App;

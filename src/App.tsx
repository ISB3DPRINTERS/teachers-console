import Auth from "./pages/auth";
import { Account } from "./pages/account";

export const App = () => {
    const token = localStorage.getItem("auth-token");

    return (
        <Account></Account>
        /*
        <div className="container" style={{ padding: "50px 0 100px 0" }}>
            {!token ? (
                <Auth />
            ) : (
                <Account token={token} />
            )}
        </div>*/
    );
}

export default App;

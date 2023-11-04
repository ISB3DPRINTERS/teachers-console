import Auth from "./pages/auth";
import { Account } from "./pages/account";

export const App = () => {
    const token = localStorage.getItem("auth-token");

    return (
        //Comment the following div to disable the login page
        <div className="container" style={{ padding: "50px 0 100px 0" }}>
            {!token ? (
                <Auth />
            ) : (
                <Account token={token} />
            )}
        </div>
        //uncomment following line to show admin page (FOR TESTING ONLY)
        // <Account></Account>
    );
}

export default App;

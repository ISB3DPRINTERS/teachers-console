import { FC } from "react";

export const Account: FC<{ token: string }> = () => {
    return (
        <div>
            <h1>Teachers' Console</h1>
            <br></br>
            <p>Welcome to the ISB3DPRINTERS teachers' console, from here you can find many administrator tools and commands</p>
            <br></br>
            <strong>Never give students access to this page</strong>
            <br></br>
            <button>Reset Passkeys</button>
            <br></br>
            <h3>Info Table</h3>
        </div>
    );
}

import { FC } from "react";
import getinfo from "../getinfo"
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
            <br></br>
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
            <button>Reset Keys</button>
            <p>Only use if keys are compromised/students are misusing access</p>
        </div>
    );
}

import { FC } from "react";
import * as React from 'react'
import getinfo from "../api/getinfo"
import { resethandler } from "../api/buttonfuncs";

export const Account: FC<{ token: string }> = () => {

    function handleSubmit(event: React.SyntheticEvent<HTMLFormElement>) {
        event.preventDefault();
        const form = event.currentTarget;
        const formElements = form.elements as typeof form.elements & {
            gradeInput: { value: number };
        };
        resethandler(formElements.gradeInput.value);
    }

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
            <form onSubmit={handleSubmit}>
                <div>
                <label htmlFor="gradeInput">Grade:</label>
                <input id="gradeInput" type="number" />
                </div>
                <button type="submit">Submit</button>
            </form>
        </div>
    );
}
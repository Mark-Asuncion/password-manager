import { useCallback, useRef, useState } from "react";
import { MInput } from "./input";
import { update } from "../../utils/invoker";
import { InputPassword } from "./password";
import { AddNo, AddOk, Copy, MDelete } from "./actions";

interface RowsProps {
    children: JSX.Element | JSX.Element[],
}

export function Rows(props: RowsProps) {
    return (
        <div className="text-white w-full flex flex-col gap-1">
            <div className="w-full grid grid-cols-4 text-center text-xl mt-2 gap-2">
                <h1>Username</h1>
                <h1>Link</h1>
                <h1>Password</h1>
                <h1>Actions</h1>
            </div>
            <div className='border-b border-neutral-600 mb-2'></div>
            {props.children}
        </div>
    )
}

interface RowProps {
    id?:      number,
    username: string,
    link:     string,
    password: string
}

export function Row(props: RowProps) {
    const [username, setUsername] = useState(props.username);
    const [link, setLink]         = useState(props.link);
    const [password, setPassword] = useState(props.password);
    const updator = useCallback(() => {
        update([username, link], props.id);
    }, [username, link, props]);

    return (
        <div id={(props.id)? props.id:0} className="w-full grid grid-cols-4 my-1 gap-2">
            <MInput
                value={username}
                onChange={(e) => {
                    setUsername(e);
                }}
                placeholder="Username"
                updator={updator}
            />
            <MInput
                value={link}
                onChange={(e) => {
                    setLink(e);
                }}
                placeholder="Link"
                updator={updator}
            />
            <InputPassword
                value={password}
                onChange={(e) => {
                    setPassword(e);
                }}
                placeholder="Password"
                updator={updator}
            />
            <div className="flex flex-row items-center m-auto">
                <Copy
                    onClick={(_) => {
                        navigator.clipboard.writeText(password);
                    }}
                />

                <MDelete
                    onClick={(_) => {
                        console.log("delete");
                        window.location.reload();
                    }}
                />
            </div>
        </div>
    )
}


export function AddRow() {
    const [username, setUsername] = useState("");
    const [link, setLink]         = useState("");
    const [password, setPassword] = useState("");

    return (
        <div
            className="w-full grid grid-cols-4 my-1 gap-2
                opacity-95 rounded-md">
            <MInput
                value={username}
                onChange={(e) => {
                    setUsername(e);
                }}
                placeholder="Username"
                updator={() => {}}
            />
            <MInput
                value={link}
                onChange={(e) => {
                    setLink(e);
                }}
                placeholder="Link"
                updator={() => {}}
            />
            <InputPassword
                value={password}
                onChange={(e) => {
                    setPassword(e);
                }}
                placeholder="Password"
                updator={() => {}}
            />
            <div className="flex flex-row items-center m-auto">
                <AddNo
                    onClick={(_) => {
                        console.log("cancel");
                    }}
                />

                <AddOk
                    onClick={(_) => {
                        console.log("add");
                    }}
                />
            </div>
        </div>
    )
}

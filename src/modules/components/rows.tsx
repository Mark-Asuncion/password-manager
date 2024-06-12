import { useCallback, useState } from "react";
import { MInput } from "./input";
import { Account, update_account } from "../../utils/invoker";
import { InputPassword } from "./password";
import { AddNo, AddOk, Copy, MDelete } from "./actions";

interface RowsProps {
    children: JSX.Element | JSX.Element[],
}

export function Rows(props: RowsProps) {
    return (
        <div className="text-white w-full flex flex-col gap-1">
            <div className="w-full grid grid-cols-4 text-center font-bold text-xl mt-2 gap-2">
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
    const updator = useCallback((_update: Account) => {
        update_account({
            username: ( _update.username )? undefined:username,
            link: (_update.link)? undefined:link,
            password: (_update.password)? undefined:password
        },_update);
    }, [username, link, password, props]);

    return (
        <div id={(props.id)? props.id + "":""}
            className="w-full grid grid-cols-4 my-1 gap-2">
            <MInput
                value={username}
                onChange={(e) => {
                    setUsername(e);
                }}
                placeholder="Username"
                updator={() => updator({username})}
            />
            <MInput
                value={link}
                onChange={(e) => {
                    setLink(e);
                }}
                placeholder="Link"
                updator={() => updator({link})}
            />
            <InputPassword
                value={password}
                onChange={(e) => {
                    setPassword(e);
                }}
                placeholder="Password"
                updator={() => updator({password})}
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


interface AddRowProps {
    onCancel: () => void,
    onOk:     (v: Account) => void
}
export function AddRow(props: AddRowProps) {
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
                        props.onCancel()
                    }}
                />
                <AddOk
                    onClick={(_) => {
                        if ( username.length === 0 || password.length === 0 ) {
                            return;
                        }
                        props.onOk({
                            username,
                            link,
                            password
                        })
                    }}
                />
            </div>
        </div>
    )
}

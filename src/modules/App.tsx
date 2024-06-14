import React, { useCallback, useEffect } from "react";
import Searchbar from "./components/searchbar";
import MoreSettings from "./components/moresettings";
import Add from "./components/add";
import Save from "./components/save";
import { Modal } from "./components/modal";
import { Row, Rows, AddRow } from "./components/rows";
import { Account, add_account, append_account, get_accounts, save, create_archive_tar } from "../utils/invoker";
import { Loading } from "./components/loading";
import { mopen, exportDialog } from "../utils/dialog";
import { Error, Notifications, Notify } from "./components/notification";

interface NotifyInfo {
    content: string,
    id:      string,
}

export default function App() {
    const [searchterm, setSearchTerm] = React.useState("");
    const [accounts,setAccounts] = React.useState<Account[]>([]);
    const [showAddRow, setShowAddRow] = React.useState(false);
    const [showLoading,setShowLoading] = React.useState(false);
    const [reload, setReload] = React.useState(true);
    const [notifications, setNotifications] = React.useState<NotifyInfo[]>([]);
    const [err, setErr] = React.useState({
        show: false,
        title: "Error",
        content: ""
    });

    const setNotifHelper = useCallback((v: NotifyInfo) => {
        setNotifications((prev) => {
            if (prev.length !== 0 && prev[prev.length-1].id === v.id) {
                return prev;
            }
            prev.push(v);
            return prev.slice();
        });
    }, []);

    const options = [
        {
            name: "Append",
            cb: async () => {
                setShowLoading(true);
                try {
                    let path = await mopen();
                    if (path)
                        await append_account(path);
                }
                catch (e) {
                    setErr({
                        show: true,
                        title: "Error Occured Appending",
                        content: `${e}`
                    });
                }
                setReload(true);
                setTimeout(() => {
                    setShowLoading(false)
                }, 500);
            }
        },
        {
            name: "Export",
            cb: async () => {
                try {
                    let path = await exportDialog();
                    if (path) {
                        const tarFileP = await create_archive_tar(path);
                        const d = new Date(Date.now());
                        let id = `${d.getUTCHours()}:${d.getUTCMinutes()}:${d.getUTCSeconds()}.${d.getUTCMilliseconds()}`;
                        setNotifHelper({
                            id,
                            content: `Export Done successfully on ${tarFileP}`
                        });
                    }
                }
                catch (e) {
                    setErr({
                        show: true,
                        title: "Error Occured Appending",
                        content: `${e}`
                    });
                }
            }
        },
    ];

    useEffect(() => {
        if (reload) {
            let query = undefined;
            if (searchterm.trim().length != 0) {
                const st = searchterm.trim()
                query = {
                    username: st,
                    link:     st
                };
            }
            // console.log(query);
            get_accounts(query)
                .then((v) => {
                    setAccounts(v);
                    // console.log(accounts);
                });
            setReload(false);
        }
    }, [reload]);

    return (
        <div className="w-[80%] m-auto my-6">
            {
                (notifications.length !== 0)?
                <Notifications>
                    {
                        notifications.map((v) => {
                            return (
                                <Notify
                                    key={v.id}
                                    content={v.content}
                                    onClose={() => {
                                        setNotifications(prev => {
                                            return prev.filter((j) => {
                                                return v.id !== j.id;
                                            });
                                        })
                                    }}
                                />
                            )
                        })
                    }
                </Notifications>:<></>
            }
            <div className="w-full m-auto my-2 flex flex-row items-center gap-2">
                <MoreSettings
                    options={options}
                    onClick={(_) => {}}
                />
                <Searchbar
                    value={searchterm}
                    classContainer='grow'
                    className='w-full'
                    placeholder='Search'
                    onChange={async (e) => {
                        const v = e.target.value;
                        setSearchTerm(v);
                        setReload(true);
                    }}
                    shortcut="/"
                />
                <Add onClick={(_) => {
                    setShowAddRow(true);
                }} />
                <Save onClick={async (_) => {
                    try {
                        await save();
                        setShowLoading(true);
                    }
                    catch (e) {
                        console.log(e);
                        setErr({
                            show: true,
                            title: "Error Occured When Saving",
                            content: `${e}`
                        });
                    }
                    setTimeout(() => {
                        setShowLoading(false)
                    }, 500);
                }} />
            </div>
            <Rows>
                <>
                   {
                        (showAddRow)?
                            <AddRow
                                onOk={async (v) => {
                                    try {
                                        await add_account(v);
                                        setShowAddRow(false);
                                    }
                                    catch (e) {
                                        setErr({
                                            show: true,
                                            title: "Duplicate",
                                            content: `${e}`
                                        })
                                    }
                                    setReload(true);
                                }}
                                onCancel={() => {
                                    setShowAddRow(false);
                                }}
                            />:<></>
                    }
                    {
                        accounts.map((v, i) => {
                            return (
                                <Row
                                    key={`${v.username}-${v.link}`}
                                    rowN={i+1}
                                    username={v.username!}
                                    link={v.link!}
                                    password={v.password!}
                                    onUpdateErr={(e) => {
                                        setReload(true);
                                        setErr({
                                            show: true,
                                            title: "Update",
                                            content: `${e}`
                                        });
                                    }}
                                    onDelete={(id) => {
                                        setReload(true);
                                        setNotifHelper({
                                            id: `${id}-delete`,
                                            content: "Row Deleted"
                                        });
                                    }}
                                    onCopyPass={(id) => {
                                        setNotifHelper({
                                            id: `${id}-copypass`,
                                            content: "Password Copied to Clipboard"
                                        });
                                    }}
                                />
                            )
                        })
                    }
                </>
            </Rows>
            <>
            {
                (showLoading) ?
                    <Modal onClick={(_) => {
                        // setShowLoading(false);
                    }} >
                        <Loading />
                    </Modal>:<></>
            }
            {
                (err.show)?
                    <Modal onClick={(_) => {
                        setErr(prev => {
                                return {
                                    ...prev,
                                    show: false
                                };
                            });
                        }}>
                            <Error
                                title={err.title}
                                content={err.content}
                                onClose={() => {
                                    setErr(prev => {
                                        return {
                                            ...prev,
                                            show: false
                                        };
                                    });
                                }}
                            />
                        </Modal>:<></>
                }
            </>
        </div>
    );
}

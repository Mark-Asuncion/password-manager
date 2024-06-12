import { momentum } from 'ldrs';


export function Loading() {
    momentum.register();
    return (
        <div className="absolute w-max h-max top-[25%] left-0 right-0 mx-auto">
            <l-momentum
                size="75"
                speed="1.1"
                color="white"
            ></l-momentum>
        </div>
    )
}

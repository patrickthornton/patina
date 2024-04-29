import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies, params }) => {
    let user_name = cookies.get("user_name");
    return {
        name: params.name,
        following: await fetch(
            `http://127.0.0.1:8000/user/${user_name}/follows/${params.name}`,
        )
            .then((res) => {
                if (!res.ok || user_name === "") {
                    throw new Error("HTTP error " + res.status);
                }
                return true;
            })
            .catch((error) => {
                console.error("Error:", error);
                return false;
            }),
        followee: await fetch(
            `http://127.0.0.1:8000/get/user/from/name/${params.name}`,
        )
            .then((res) => {
                if (!res.ok) {
                    throw new Error("HTTP error " + res.status);
                }
                return res.json();
            })
            .catch((error) => {
                console.error("Error:", error);
            }),
    };
};

<script lang="ts">
    import { colorFromHue, now, hueFromTime } from "$lib/hue.svelte";
    import { user_id, user_name } from "../../stores/user.js";
    import { goto } from "$app/navigation";

    let time = now();
    setInterval(() => {
        time = now();
    }, 200);

    $: starting_hue = colorFromHue(hueFromTime(time));
    $: ending_hue = colorFromHue(hueFromTime(time + 90));

    let name = "";

    async function login() {
        // if name is blank or contains whitespace, reject it
        if (!name || /\s/g.test(name)) {
            return;
        }
        await fetch(`http://127.0.0.1:8000/get/user/from/name/${name}`)
            .then((res) => {
                if (res.ok) {
                    return res.json();
                } else {
                    throw new Error("user not found");
                }
            })
            .then((data) => {
                user_id.set(data);
                user_name.set(name);
                goto("/");
            })
            .catch(async (err) => {
                await fetch("http://127.0.0.1:8000/post/user", {
                    method: "POST",
                    headers: {
                        "Content-Type": "text/plain",
                    },
                    body: JSON.stringify({ name }),
                })
                    .then((res) => {
                        if (res.ok) {
                            return res.json();
                        } else {
                            throw new Error("user not added");
                        }
                    })
                    .then((data) => {
                        user_id.set(data.user_id);
                        user_name.set(data.name);
                        goto("/");
                        console.log("user id" + data.user_id);
                    })
                    .catch((err) => {
                        console.error(err);
                    });
            });
        name = "";
    }
</script>

<div
    class="gradient"
    style="background: linear-gradient(0, {starting_hue}, {ending_hue})"
>
    <div class="login">
        <h1>welcome!</h1>
        <h2>what's your name?</h2>
        <form class="form" on:submit|preventDefault={login}>
            <input
                type="text"
                class="name"
                bind:value={name}
                placeholder="i am known as..."
            />
        </form>
    </div>
</div>

<style>
    .gradient {
        width: 100vw;
        height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .login {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        text-align: center;
        color: white;
        border-radius: 1rem;
        padding: 1rem;
        border: 1px solid white;
        width: 30%;
        height: 50%;
        text-decoration: none;
    }

    h1 {
        font-size: 3rem;
        font-weight: bold;
        margin-bottom: 5rem;
    }

    h2 {
        font-size: 2rem;
        font-weight: 400;
        margin-bottom: 1rem;
    }

    form {
        width: 100%;
    }

    .name {
        width: 30%;
        height: 2rem;
        font-size: 1.5rem;
        padding: 0.5rem;
        border-radius: 0.5rem;
        border: 1px solid white;
        background-color: transparent;
        color: white;
        margin-bottom: 3rem;
    }
</style>

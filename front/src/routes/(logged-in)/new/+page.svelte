<script lang="ts">
    import Post from "$lib/interfaces.svelte";
    import Slider from "$lib/slider.svelte";
    import { colorFromHue } from "$lib/hue.svelte";
    import { user_name } from "../../../stores/user.js";
    import { goto } from "$app/navigation";

    let post_text = "";
    let post_hue = 0;

    let newpost: Post;
    async function post() {
        let author: string = $user_name;
        let text: string = post_text;
        let hue: number = post_hue;
        let reply_to = null;

        if (!text) {
            return;
        }

        await fetch("http://127.0.0.1:8000/post/post", {
            method: "POST",
            headers: {
                "Content-Type": "text/plain",
            },
            body: JSON.stringify({ author, text, hue, reply_to }),
        }).catch((err) => console.error(err));

        // reset the input after adding a post
        post_text = "";
        goto("/");
    }
</script>

<div class="backdrop" style="background-color: {colorFromHue(post_hue)}">
    <form class="workbench" on:submit|preventDefault={post}>
        <h2>new post</h2>
        <Slider bind:hue={post_hue} />
        <input
            type="textarea"
            class="post"
            bind:value={post_text}
            placeholder="i was just thinking..."
        />
        <button type="submit" class="submit">post!</button>
    </form>
</div>

<style>
    .backdrop {
        height: 100vh;
        width: 100vw;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .workbench {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        text-align: center;
        color: white;
        border-radius: 1rem;
        padding: 1rem;
        border: 1px solid white;
        width: 20%;
        height: 40%;
        text-decoration: none;
    }

    .post {
        width: 80%;
        height: 100%;
        margin: 1rem;
        border-radius: 1rem;
        border: 1px solid white;
        padding: 1rem;
        background-color: transparent;
        color: white;
        font-family: "Spectral", serif;
        font-size: 12pt;
        text-wrap: unrestricted;
        overflow: hidden;
        resize: vertical;
    }

    .submit {
        width: 20%;
        height: 10%;
        margin: 1rem;
        border-radius: 0.5rem;
        border: 1px solid white;
        background-color: rgb(255, 255, 255, 0.15);
        color: white;
        font-family: "Spectral", serif;
        font-size: 12pt;
    }

    .submit:hover {
        background-color: rgb(255, 255, 255, 0.3);
    }
</style>

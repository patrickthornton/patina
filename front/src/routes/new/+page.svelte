<script lang="ts">
    import Post from "$lib/interfaces.svelte";
    import { colorFromHue } from "$lib/hue.svelte";

    let posttxt = "";
    let posthue: number;

    let newpost: Post;
    async function newPost() {
        let author: string = "shodslip";
        let text: string = posttxt;
        let hue: number = posthue;

        await fetch("http://127.0.0.1:8000/post/post", {
            method: "POST",
            headers: {
                "Content-Type": "text/plain",
            },
            body: JSON.stringify({ author, text, hue }),
        }).catch((err) => console.error(err));

        // reset the input after adding a post
        posttxt = "";
    }
</script>

<div class="slider" style="background-color: {colorFromHue(360 - posthue)};">
    <input type="range" min="0" max="360" bind:value={posthue} />

    current hue: {posthue}
</div>

<h1>new post</h1>
<input bind:value={posttxt} placeholder="new post" />
<!-- <input bind:value={posthue} placeholder="choose color" /> -->
<button on:click={newPost}>Add Post</button>

<style>
    .slider {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        color: white;
        border-radius: 0.5rem;
        padding: 1rem;
    }
</style>

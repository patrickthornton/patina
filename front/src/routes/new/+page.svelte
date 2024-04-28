<script lang="ts">
    let posttxt = "";
    import { posts } from "../../stores/posts.js";
    import { colorFromHue } from "$lib/hue.svelte";
    interface Post {
        id: number;
        author: string;
        text: string;
        hue: number;
    }
    let posthue: number;

    let newpost: Post;
    function addNewPost() {
        const newPost: Post = {
            id: (Math.random() * 500) | 50,
            author: "me",
            text: posttxt,
            hue: posthue,
        };

        posts.update((allPosts) => {
            allPosts.push(newPost);
            return allPosts;
        });

        // Reset the input after adding a post
        posttxt = "";
    }
</script>

<div class="slider" style="background-color: {colorFromHue(360 - posthue)};">
    <input type="range" min="0" max="360" bind:value={posthue} />

    current hue: {posthue}
</div>

<h1>new post</h1>
<input bind:value={posttxt} placeholder="new post" />
<input bind:value={posthue} placeholder="choose color" />
<button on:click={addNewPost}>Add Post</button>

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

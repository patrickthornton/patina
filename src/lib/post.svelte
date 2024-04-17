<script lang="ts">
    import { colorFromHue } from "./hue.svelte";

    interface Post {
        id: number;
        author: string;
        text: string;
        hue: number;
    }

    export let post: Post;
</script>

<div
    class="post"
    style="
    background-color: {colorFromHue(post.hue)};
    "
>
    <p class="author">{post.author}</p>
    <p class="text">{post.text}</p>
</div>

<style>
    .post {
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        padding-left: 1rem;
        padding-right: 1rem;
    }

    .author {
        color: #ddd;
        align-self: flex-end;
    }

    .text {
        color: white;
        font-size: 1.5rem;
    }
</style>

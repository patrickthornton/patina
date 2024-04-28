<script lang="ts">
    import { colorFromHue } from "./hue.svelte";
    import Post from "./interfaces.svelte";
    import Like from "./like.svelte";

    export let post: Post;
</script>

<div class="post" style="background-color: {colorFromHue(post.hue)};">
    <div class="content">
        <a href="/user/{post.author}" class="author">{post.author}</a>
        <p class="text">{post.text}</p>
        <Like post={post.post_id} />
    </div>
</div>

<style>
    .post {
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        align-items: center;
        padding-left: 1rem;
        padding-right: 1rem;
        padding-top: 0.5rem;
        padding-bottom: 0.2rem;
    }

    .content {
        width: 45%;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    .author {
        color: #eee;
        font-weight: bold;
        align-self: flex-end;
        text-decoration: none;
    }

    .text {
        color: white;
        font-size: 1.5rem;
        font-weight: 300;
        line-height: 1.3;
    }
</style>

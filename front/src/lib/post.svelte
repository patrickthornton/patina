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
    @import url("https://fonts.googleapis.com/css2?family=EB+Garamond:ital,wght@0,400..800;1,400..800&family=Spectral:ital,wght@0,200;0,300;0,400;0,500;0,600;0,700;0,800;1,200;1,300;1,400;1,500;1,600;1,700;1,800&display=swap");

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
        font-family: "Spectral", serif;
        text-decoration: none;
    }
    .text {
        color: white;
        font-size: 1.5rem;
        font-family: "Spectral", serif;
        font-weight: 300;
        line-height: 1.3;
    }
</style>

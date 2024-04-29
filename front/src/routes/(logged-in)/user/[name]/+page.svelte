<script lang="ts">
    import { onMount } from "svelte";
    import Timeline from "$lib/timeline.svelte";
    import FollowButton from "$lib/button_follow.svelte";
    import Post from "$lib/interfaces.svelte";
    import { colorFromHue } from "$lib/hue.svelte";

    export let data;
    let author: number = data.name;
    let posts: Post[] = data.posts;

    let starting_hue = 0;
</script>

<div
    class="author-header"
    style="background-color: {colorFromHue(360 - starting_hue)};"
>
    <h2>{author}</h2>
    <FollowButton />
</div>

<Timeline {posts} />

<style>
    .author-header {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        font-family: Spectral, serif;
        width: 100%;
        font-weight: bold;
        text-align: center;
    }

    .follow-button {
        width: 700px;
    }
</style>

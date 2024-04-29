<script lang="ts">
    import { onMount } from "svelte";
    import Timeline from "$lib/timeline.svelte";
    import Title from "$lib/title.svelte";
    import Post from "$lib/interfaces.svelte";
    import { now } from "$lib/hue.svelte";

    export let data;
    let posts: Post[] = data.posts;
</script>

<Timeline {posts} />

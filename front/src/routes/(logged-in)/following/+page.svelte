<script lang="ts">
    import Timeline from "$lib/timeline.svelte";
    import Post from "$lib/interfaces.svelte";

    export let data;
    let posts: Post[] = data.posts;
</script>

<Timeline {posts} />

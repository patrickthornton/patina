<script lang="ts">
    import Wheel from "$lib/colorwheel.svelte";
    import Post from "$lib/interfaces.svelte";

    export let data;
    let posts: Post[] = data.posts;
</script>

<Wheel {posts} />

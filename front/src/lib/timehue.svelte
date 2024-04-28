<script lang="ts">
    var current_time = Math.floor(new Date().getTime() / 1000);
    setInterval(() => {
        current_time = Math.floor(new Date().getTime() / 1000);
    }, 200);
    $: starting_hue = timehue(current_time);

    function timehue(now: number) {
        return now % 360;
    }
</script>

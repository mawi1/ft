<script lang="ts">
  type Props = {
    id: string;
    label: string;
    value: number;
    min: string;
    max: string;
    step: string;
  };

  let {id, label, value = $bindable(), min, max, step}: Props  = $props();
</script>

<div class="w-full flex flex-col items-start mt-2">
  <div>
    <label for={id}>{label}:</label>
    <output>{value}</output>
  </div>
  <input
    class="w-full"
    id={id}
    type="range"
    bind:value
    {min}
    {max}
    {step}
  />
</div>

<script lang="ts">

    // I'll let you handle the rest since idk what you mean by your shortcut.
    type textSize = 'large' | 'medium' | 'small'
    type textType = 'display' | 'headline' | 'title' | 'label' | 'body'

    type StyledText = `${textType}-${textSize}`;

    const magic:StyledText = "label-small";

	//export let textType;
	//export let textSize;
</script>


<div id="text" class="{$$props.class}">
	<slot />
</div>

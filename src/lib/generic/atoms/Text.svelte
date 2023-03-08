<script lang="ts">
	enum TextType {
		Display,
		Headline,
		Title,
		Label,
		Body
	}
	enum TextSize {
		Large,
		Medium,
		Small
	}

	//export let textType;
	//export let textSize;

    let className = "";
    export { className as class};

    /*
    let className = "";
	
        switch(textType.type) {
        case TextType.Display:
            className += "display-";
            break;
        case TextType.Headline:
        className += "headline-";
            break;
        case TextType.Title:
        className += "title-";
            break;
        case TextType.Label:
        className += "label-";
            break;
        case TextType.Body:
        className += "body-";
            break;
    };
    
   
    switch(textSize) {
        case TextSize.Large:
        className += "large";
            break;
        case TextSize.Medium:
        className += "medium";
            break;
        case TextSize.Small:
        className += "small";
            break;
    }
    */
</script>


<div id="text" class={className}>
	<slot />
</div>

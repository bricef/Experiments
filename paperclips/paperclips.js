



let btn = document.getElementById('btnQcompute')

let chips = Array.from(document.getElementsByClassName('qChip'))


let interval = setInterval(()=>{
	let t = chips.map(c => c.style.opacity).map(parseFloat).reduce( (x,y) => x+y );
	if (t > 0){
		btn.click()
	}

}, 10)

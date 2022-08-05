async function init() {
    let rustApp = null

    try {
        //import webassembly
        rustApp = await import('../pkg')
    } catch (err) {
        console.error(err)
        return
    }

    console.log(rustApp)

    const input = document.getElementById('upload')
    const fileReader = new FileReader() // stores file on our JS app on the browser

    //  This event is called once the file is completely loaded
    fileReader.onloadend = () => {
        let base64 = fileReader.result.replace(
            /^data:image\/(png|jpeg|jpg);base64,/, ''
        )
        
        let image_data_url = rustApp.grayscale(base64)
        document.getElementById('new-img').setAttribute('src', image_data_url)
    }

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0])
    })
}

init()
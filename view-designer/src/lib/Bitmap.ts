export var Bitmap = {
    toArrayBuffer: function(pixelData: any, width: number, height: number) {
        var w = width,
            h = height,
            data32 = new Uint32Array(pixelData),
            stride = Math.floor((24 * w + 31) / 32) * 4, // row length for 24 bit bitmap incl. padding
            pixelArraySize = stride * h, // total bitmap size
            fileLength = 54 + pixelArraySize, // header size + bitmap
            file = new ArrayBuffer(fileLength),
            view = new DataView(file),
            pos = 0,
            x,
            y = h - 1,
            p,
            s = 0,
            v;

        // write file header
        setU16(0x4d42); // BM
        setU32(fileLength); // total length
        pos += 4; // skip unused fields
        setU32(0x36); // offset to pixels

        // DIB header
        setU32(40); // header size
        setU32(w);
        setU32(h >>> 0); // negative = top-to-bottom
        setU16(1); // 1 plane
        setU16(24); // 24-bits (RGB)
        setU32(0); // no compression (BI_RGB, 0)
        setU32(pixelArraySize); // bitmap size incl. padding (stride x height)
        setU32(2835); // pixels/meter h (~72 DPI x 39.3701 inch/m)
        setU32(2835); // pixels/meter v
        pos += 8; // skip color/important colors

        // bitmap data, change order of ABGR to BGR
        while (y > 0) {
            p = 0x36 + y * stride; // offset + stride x height
            x = 0;
            while (x < w * 3) {
                v = data32[s++]; // get ABGR
                view.setUint8(p + x, (v >> 16) & 0xff); // set R
                view.setUint8(p + x + 1, (v >> 8) & 0xff); // set G
                view.setUint8(p + x + 2, v & 0xff); // set B
                x += 3;
            }
            y--;
        }

        return file;

        // helper method to move current buffer position
        function setU16(data: any) {
            view.setUint16(pos, data, true);
            pos += 2;
        }
        function setU32(data: any) {
            view.setUint32(pos, data, true);
            pos += 4;
        }
    },

    fromCanvas: function(canvas: any) {
        var buffer = new Uint8Array(this.toArrayBuffer(canvas.getContext("2d").getImageData(0, 0, canvas.width, canvas.height).data.buffer, canvas.width, canvas.height)),
            bs = "",
            i = 0,
            l = buffer.length;
        while (i < l) bs += String.fromCharCode(buffer[i++]);
        return "data:image/bmp;base64," + btoa(bs);
    },

    fromPixelData: function(pixelData: any, width: number, height: number) {
        var buffer = new Uint8Array(this.toArrayBuffer(pixelData, width, height)),
            bs = "",
            i = 0,
            l = buffer.length;
        while (i < l) bs += String.fromCharCode(buffer[i++]);
        return "data:image/bmp;base64," + btoa(bs);
    }
};

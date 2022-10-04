import numpy as np
import cv2

class imageProc:
    def __init__(self, path) -> None:
        self._image_path = path or ""
        self._image_data = self.read_image()

    def read_image(self):
        """
        It reads the image from the path specified in the constructor and stores it in the _image_data
        variable
        """
        return cv2.imread(self._image_path, cv2.IMREAD_UNCHANGED)

    def procecss_image(self):
        """
        It takes the image data, copies it, and then loops through the image data and does some calculations
        on the data
        """
        original = np.copy(self._image_data)
        final = np.zeros_like(original);
        print(self._image_data)
        img = np.copy(original);
        unprotected = np.copy(original);
        height, width = img.shape[:2];
        #Loop for image processing
        for y in range(height):
            for x in range(width):
                #Roughness
                final[y,x,3] = img[y,x,1] #                                                                             OW G -> U A

                # Default AO

                final[y,x,1] = 255 #                                                                                    255 -> U G

                #Metallic
                scale = img[y,x,2] / 255
                if scale < 0.5:
                    scale = 0
                scaled = max(0, min(((scale - 0.5) * scale) * 2, 1)) # Calculate the Metallics & Clamp the value 

                final[y,x,2] = scaled * 255 #                                                                           OW R -> U R

        

        cv2.imwrite(self._image_path[:-4] + "-" + self._image_path[-4:], final)
        # cv2.imshow("Final", final)
        # cv2.waitKey(0)  

    # def set_roughness(self, img):
    #     pass
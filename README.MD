# Lamb essential data processing

Given chunks (30 seconds in my particular case), this programs computes some relevant acceleration parameters and gives an output.csv with them.

## Parameters calculated

The output.csv file will contain the following data in the following order:

### Average X-axis
$$A_x = \frac{1}{T} \sum_{t=1}^{T}x(t)$$

### Average Y-axis
$$A_y = \frac{1}{T} \sum_{t=1}^{T}y(t)$$

### Average Z-axis
$$A_z = \frac{1}{T} \sum_{t=1}^{T}z(t)$$

### Variation on the X axis

$$\sigma^2_x = \frac{1}{T} \sum_{t=1}^{T}\left(  x(t) - A_x \right)^2$$

### Variation on the Y axis

$$\sigma^2_y = \frac{1}{T} \sum_{t=1}^{T}\left(  y(t) - A_y \right)^2$$

### Variation on the Z axis

$$\sigma^2_z= \frac{1}{T} \sum_{t=1}^{T}\left(  z(t) - A_z \right)^2$$

### Signal Magnitude Area

$$SMA = A_x + A_y +A_z$$

### Average intensity

$$AI = \frac{1}{T} \sum_{t=1}^{T}\sqrt{x(t)^2+y(t)^2+z(t)^2}$$

## Input format
The input.csv must be located in the ``"src/input_file/"`` and must contain 900 columns,
where the first 300 correspond to the 300 samples of acceleration on the x axis, next from column 301 to 600 the samples of y acceleration and from column 601 to 900 the z accelerations.

## Todo
- [ ] Create an TUI for interacting with the user
- [ ] New parameters?
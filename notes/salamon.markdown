# salamon phd thesis (2013) notes

First, convert audio signal to mono-channel with a 44100 Hz sampling rate. Then, filter it with a 10th order IIR filter convoluted with a 2nd order Butterworth high pass filter. This roughly amplifies frequencies in the human hearing range and reduces those outside.

Compute its discrete STFT with $$X_l(k) = \sum_{n=0}^{M-1} w(n)x(n+lH)e^{-j2\pi kn/N}$$ where $x(n)$ is the audio signal, $w(n)$ is the Hann windowing function, $l$ is the frame number, $k$ is the Nth harmonic, window length $M = 2048$, FFT length $N = 8192$, and hop size $H=128$. Peaks are selected by finding the local maxima $k_m$ per frame of the normalized magnitude spectrum $X_m(k) = \frac{2}{\sum_{n=0}^{M-1} w(n)} |X(k)|$.

Phase correct the peaks by calculating the instantaneous frequency $$f = (k_m + \kappa(k_m)) f_s/N$$ where $$\kappa(k) = \frac{N}{2\pi}\psi(\phi_l(k) - \phi_{l-1}(k) - \frac{2\pi}{N}k)$$ where $\psi$ maps its argument within $[-\pi, \pi]$. Calculate the instantaneous magnitude with $$a = \frac{X_m(k_m)}{2W_h(\frac{M}{N}\kappa(k_m))}$$ where $W_h(z) = \frac{\text{sinc}(z)}{2(1 - z^2)}$.

Redefine frequencies as multiple harmonic series perceived as single tones. Compute the salience function inside the human voice frequency range at a single frame per bin $b=0,1,...,599$ by $$S(b) = \sum_{h=1}^{20}\sum_{i=1}^I E(a_i)G(b, h, f_i)a_i$$ where $f_i$ and $a_i$ are the frequency and magnitude of the peak, $E(a_i) = 1$ only if $20\log_{10}(a_{max}/a_i) < 40$ otherwise it's $0$, $G(b, h, f_i) = \cos^2(\delta\pi/2).8^{h-1}$ only if $|\delta|\le 1$ otherwise it's $0$, and $\delta=|B(f_i/h)-b|/10$ where $B(f)=1200\log_2(f/55)/10$ rounded.

Divide peaks of the salience function per frame into predominant and background peaks. Peaks below `.9 * max salience` are placed into background peaks. Peaks below `remaining peaks' mean - .9 * standard deviation` are also put into the background peaks. 

Peaks are placed into contours. Pop highest peak from the predominant set and add it to a new contour. While the next frame contains a peak that's within 80 cents of the previously found peak, pop it from the predominant set and add it to the contour. If no peak is found in the predominant peak, allow adding contours for 100 ms from the background peaks. Then, repeat this process going backwards in frames.

Filter out all contours with mean salience less than `mean salience of all contours - .9 * standard deviation`. Contours with a pitch deviation above 40 cents will have their mean salience multiplied by 1.5 (likely belongs to a human voice). Contours with vibrato detected will have their mean salience multipled by 3. Vibrato is detected by applying a FFT to the contour's pitch sequence - mean and finding whether there is a prominent peak between 5-8 Hz.

Calculate melody pitch mean ($P(n)$) per frame as the mean of the pitch values of all contours present in the frame (1). Smooth $P(n)$ using a 5 second sliding mean filter (2). Delete contours with octave errors (3). Two contours are octave duplicates if their overlapping mean distance is within 1200 +/- 50 cents. If one of the duplicates has less than half the total salience, remove it. Otherwise, the contour closest to the melody pitch mean is selected.  But if both contours are half an octave or more further from the melody pitch mean, keep the contour with the greater total salience. Recompute $P(n)$ (4). Delete contours at an average distance of more than an octave from $P(n)$ (5). Recompute $P(n)$ (6). Repeat 3-6 twice (7).

Finally, if there is more than one contour present in a frame, select the peak belonging to the contour with greatest total salience.

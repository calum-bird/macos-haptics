#import <AppKit/AppKit.h>

void performHapticFeedbackMultipleTimes(int count) {
    for (int i = 0; i < count; i++) {
        [[NSHapticFeedbackManager defaultPerformer] performFeedbackPattern:NSHapticFeedbackPatternAlignment performanceTime:NSHapticFeedbackPerformanceTimeNow];
        [NSThread sleepForTimeInterval:0.5]; // add a small delay between feedbacks
    }
}
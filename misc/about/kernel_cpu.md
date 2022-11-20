# Generic Scaling Governors

CPUFreq provides generic scaling governors that can be used with all scaling drivers. As stated before, each of them implements a single, possibly parametrized, performance scaling algorithm.

Scaling governors are attached to policy objects and different policy objects can be handled by different scaling governors at the same time (although that may lead to suboptimal results in some cases).

The scaling governor for a given policy object can be changed at any time with the help of the scaling_governor policy attribute in sysfs.

Some governors expose sysfs attributes to control or fine-tune the scaling algorithms implemented by them. Those attributes, referred to as governor tunables, can be either global (system-wide) or per-policy, depending on the scaling driver in use. If the driver requires governor tunables to be per-policy, they are located in a subdirectory of each policy directory. Otherwise, they are located in a subdirectory under /sys/devices/system/cpu/cpufreq/. In either case the name of the subdirectory containing the governor tunables is the name of the governor providing them.
## performance

When attached to a policy object, this governor causes the highest frequency, within the scaling_max_freq policy limit, to be requested for that policy.

The request is made once at that time the governor for the policy is set to performance and whenever the scaling_max_freq or scaling_min_freq policy limits change after that.
powersave¶

When attached to a policy object, this governor causes the lowest frequency, within the scaling_min_freq policy limit, to be requested for that policy.

The request is made once at that time the governor for the policy is set to powersave and whenever the scaling_max_freq or scaling_min_freq policy limits change after that.
userspace¶

This governor does not do anything by itself. Instead, it allows user space to set the CPU frequency for the policy it is attached to by writing to the scaling_setspeed attribute of that policy.
schedutil¶

This governor uses CPU utilization data available from the CPU scheduler. It generally is regarded as a part of the CPU scheduler, so it can access the scheduler’s internal data structures directly.

It runs entirely in scheduler context, although in some cases it may need to invoke the scaling driver asynchronously when it decides that the CPU frequency should be changed for a given policy (that depends on whether or not the driver is capable of changing the CPU frequency from scheduler context).

The actions of this governor for a particular CPU depend on the scheduling class invoking its utilization update callback for that CPU. If it is invoked by the RT or deadline scheduling classes, the governor will increase the frequency to the allowed maximum (that is, the scaling_max_freq policy limit). In turn, if it is invoked by the CFS scheduling class, the governor will use the Per-Entity Load Tracking (PELT) metric for the root control group of the given CPU as the CPU utilization estimate (see the Per-entity load tracking LWN.net article 1 for a description of the PELT mechanism). Then, the new CPU frequency to apply is computed in accordance with the formula

    f = 1.25 * f_0 * util / max

where util is the PELT number, max is the theoretical maximum of util, and f_0 is either the maximum possible CPU frequency for the given policy (if the PELT number is frequency-invariant), or the current CPU frequency (otherwise).

This governor also employs a mechanism allowing it to temporarily bump up the CPU frequency for tasks that have been waiting on I/O most recently, called “IO-wait boosting”. That happens when the SCHED_CPUFREQ_IOWAIT flag is passed by the scheduler to the governor callback which causes the frequency to go up to the allowed maximum immediately and then draw back to the value returned by the above formula over time.

This governor exposes only one tunable:

rate_limit_us

    Minimum time (in microseconds) that has to pass between two consecutive runs of governor computations (default: 1000 times the scaling driver’s transition latency).

    The purpose of this tunable is to reduce the scheduler context overhead of the governor which might be excessive without it.

This governor generally is regarded as a replacement for the older ondemand and conservative governors (described below), as it is simpler and more tightly integrated with the CPU scheduler, its overhead in terms of CPU context switches and similar is less significant, and it uses the scheduler’s own CPU utilization metric, so in principle its decisions should not contradict the decisions made by the other parts of the scheduler.
ondemand¶

This governor uses CPU load as a CPU frequency selection metric.

In order to estimate the current CPU load, it measures the time elapsed between consecutive invocations of its worker routine and computes the fraction of that time in which the given CPU was not idle. The ratio of the non-idle (active) time to the total CPU time is taken as an estimate of the load.

If this governor is attached to a policy shared by multiple CPUs, the load is estimated for all of them and the greatest result is taken as the load estimate for the entire policy.

The worker routine of this governor has to run in process context, so it is invoked asynchronously (via a workqueue) and CPU P-states are updated from there if necessary. As a result, the scheduler context overhead from this governor is minimum, but it causes additional CPU context switches to happen relatively often and the CPU P-state updates triggered by it can be relatively irregular. Also, it affects its own CPU load metric by running code that reduces the CPU idle time (even though the CPU idle time is only reduced very slightly by it).

It generally selects CPU frequencies proportional to the estimated load, so that the value of the `cpuinfo_max_freq` policy attribute corresponds to the load of 1 (or 100%), and the value of the `cpuinfo_min_freq` policy attribute corresponds to the load of 0, unless when the load exceeds a (configurable) speedup threshold, in which case it will go straight for the highest frequency it is allowed to use (the `scaling_max_freq` policy limit).

This governor exposes the following tunables:

`sampling_rate`

```
    This is how often the governor’s worker routine should run, in microseconds.

    Typically, it is set to values of the order of 10000 (10 ms). Its default value is equal to the value of cpuinfo_transition_latency for each policy this governor is attached to (but since the unit here is greater by 1000, this means that the time represented by sampling_rate is 1000 times greater than the transition latency by default).

    If this tunable is per-policy, the following shell command sets the time represented by it to be 750 times as high as the transition latency:

    # echo `$(($(cat cpuinfo_transition_latency) * 750 / 1000)) > ondemand/sampling_rate
```

`up_threshold`

```
    If the estimated CPU load is above this value (in percent), the governor will set the frequency to the maximum value allowed for the policy. Otherwise, the selected frequency will be proportional to the estimated CPU load.
```

`ignore_nice_load`

```
    If set to 1 (default 0), it will cause the CPU load estimation code to treat the CPU time spent on executing tasks with “nice” levels greater than 0 as CPU idle time.

    This may be useful if there are tasks in the system that should not be taken into account when deciding what frequency to run the CPUs at. Then, to make that happen it is sufficient to increase the “nice” level of those tasks above 0 and set this attribute to 1.
```

`sampling_down_factor`

```

    Temporary multiplier, between 1 (default) and 100 inclusive, to apply to the sampling_rate value if the CPU load goes above up_threshold.

    This causes the next execution of the governor’s worker routine (after setting the frequency to the allowed maximum) to be delayed, so the frequency stays at the maximum level for a longer time.

    Frequency fluctuations in some bursty workloads may be avoided this way at the cost of additional energy spent on maintaining the maximum CPU capacity.
powersave_bias

    Reduction factor to apply to the original frequency target of the governor (including the maximum value used when the up_threshold value is exceeded by the estimated CPU load) or sensitivity threshold for the AMD frequency sensitivity powersave bias driver (drivers/cpufreq/amd_freq_sensitivity.c), between 0 and 1000 inclusive.

    If the AMD frequency sensitivity powersave bias driver is not loaded, the effective frequency to apply is given by

        f * (1 - powersave_bias / 1000)

    where f is the governor’s original frequency target. The default value of this attribute is 0 in that case.

    If the AMD frequency sensitivity powersave bias driver is loaded, the value of this attribute is 400 by default and it is used in a different way.

    On Family 16h (and later) AMD processors there is a mechanism to get a measured workload sensitivity, between 0 and 100% inclusive, from the hardware. That value can be used to estimate how the performance of the workload running on a CPU will change in response to frequency changes.

    The performance of a workload with the sensitivity of 0 (memory-bound or IO-bound) is not expected to increase at all as a result of increasing the CPU frequency, whereas workloads with the sensitivity of 100% (CPU-bound) are expected to perform much better if the CPU frequency is increased.

    If the workload sensitivity is less than the threshold represented by the powersave_bias value, the sensitivity powersave bias driver will cause the governor to select a frequency lower than its original target, so as to avoid over-provisioning workloads that will not benefit from running at higher CPU frequencies.

```
## conservative

This governor uses CPU load as a CPU frequency selection metric.

It estimates the CPU load in the same way as the ondemand governor described above, but the CPU frequency selection algorithm implemented by it is different.

Namely, it avoids changing the frequency significantly over short time intervals which may not be suitable for systems with limited power supply capacity (e.g. battery-powered). To achieve that, it changes the frequency in relatively small steps, one step at a time, up or down - depending on whether or not a (configurable) threshold has been exceeded by the estimated CPU load.

This governor exposes the following tunables:

`freq_step`

```
    Frequency step in percent of the maximum frequency the governor is allowed to set (the scaling_max_freq policy limit), between 0 and 100 (5 by default).

    This is how much the frequency is allowed to change in one go. Setting it to 0 will cause the default frequency step (5 percent) to be used and setting it to 100 effectively causes the governor to periodically switch the frequency between the scaling_min_freq and scaling_max_freq policy limits.

```

`down_threshold`

```
    Threshold value (in percent, 20 by default) used to determine the frequency change direction.

    If the estimated CPU load is greater than this value, the frequency will go up (by freq_step). If the load is less than this value (and the sampling_down_factor mechanism is not in effect), the frequency will go down. Otherwise, the frequency will not be changed.
sampling_down_factor

    Frequency decrease deferral factor, between 1 (default) and 10 inclusive.

    It effectively causes the frequency to go down sampling_down_factor times slower than it ramps up.
```


//! Component for the ADC on the imix board.
//!
//! This provides one Component, AdcComponent, which implements
//! a userspace syscall interface to the SAM4L ADC. It provides
//! 6 ADC channels, AD0-AD5.
//!
//! Usage
//! -----
//! ```rust
//! let adc = AdcComponent::new().finalize(());
//! ```

// Author: Philip Levis <pal@cs.stanford.edu>
// Last modified: 6/20/2018

#![allow(dead_code)] // Components are intended to be conditionally included

use capsules::adc;
use kernel::capabilities;
use kernel::component::Component;
use kernel::create_capability;
use kernel::static_init;

pub struct AdcComponent {
    board_kernel: &'static kernel::Kernel,
}

impl AdcComponent {
    pub fn new(board_kernel: &'static kernel::Kernel) -> AdcComponent {
        AdcComponent {
            board_kernel: board_kernel,
        }
    }
}

impl Component for AdcComponent {
    type StaticInput = ();
    type Output = &'static adc::Adc<'static, sam4l::adc::Adc>;

    unsafe fn finalize(&mut self, _s: Self::StaticInput) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
        let adc_channels = static_init!(
            [&'static sam4l::adc::AdcChannel; 6],
            [
                &sam4l::adc::CHANNEL_AD1, // AD0
                &sam4l::adc::CHANNEL_AD2, // AD1
                &sam4l::adc::CHANNEL_AD3, // AD2
                &sam4l::adc::CHANNEL_AD4, // AD3
                &sam4l::adc::CHANNEL_AD5, // AD4
                &sam4l::adc::CHANNEL_AD6, // AD5
            ]
        );
        let adc = static_init!(
            adc::Adc<'static, sam4l::adc::Adc>,
            adc::Adc::new(
                &mut sam4l::adc::ADC0,
                self.board_kernel.create_grant(&grant_cap),
                adc_channels,
                &mut adc::ADC_BUFFER1,
                &mut adc::ADC_BUFFER2,
                &mut adc::ADC_BUFFER3
            )
        );
        sam4l::adc::ADC0.set_client(adc);

        adc
    }
}

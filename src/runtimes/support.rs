// The MIT License (MIT)
// Copyright © 2021 Aukbit Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::config::CONFIG;
pub type ChainPrefix = u16;
pub type ChainTokenSymbol = String;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedRuntime {
    Polkadot,
    Kusama,
    Westend,
    Paseo,
}

impl SupportedRuntime {
    pub fn people_runtime(&self) -> Option<SupportedParasRuntime> {
        match &self {
            Self::Kusama => Some(SupportedParasRuntime::PeopleKusama),
            _ => None,
        }
    }
}

impl From<ChainPrefix> for SupportedRuntime {
    fn from(v: ChainPrefix) -> Self {
        match v {
            0 => Self::Polkadot,
            2 => Self::Kusama,
            42 => Self::Westend,
            _ => unimplemented!("Chain prefix not supported"),
        }
    }
}

impl From<ChainTokenSymbol> for SupportedRuntime {
    fn from(v: ChainTokenSymbol) -> Self {
        match v.as_str() {
            "DOT" => Self::Polkadot,
            "KSM" => Self::Kusama,
            "WND" => Self::Westend,
            "PAS" => Self::Paseo,
            _ => unimplemented!("Chain unit not supported"),
        }
    }
}

impl std::fmt::Display for SupportedRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Polkadot => write!(f, "Polkadot"),
            Self::Kusama => write!(f, "Kusama"),
            Self::Westend => write!(f, "Westend"),
            Self::Paseo => write!(f, "Paseo"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SupportedParasRuntime {
    PeopleKusama,
}

impl SupportedParasRuntime {
    pub fn default_rpc_url(&self) -> String {
        let config = CONFIG.clone();
        match &self {
            Self::PeopleKusama => config.substrate_people_ws_url,
        }
    }
}

impl std::fmt::Display for SupportedParasRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PeopleKusama => write!(f, "People Kusama"),
        }
    }
}

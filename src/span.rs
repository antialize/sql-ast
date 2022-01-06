// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Byte span of ast fragment
pub type Span = core::ops::Range<usize>;

/// Return or compute byte span of ast fragment
pub trait Spanned {
    /// Return or compute byte span of ast fragment
    fn span(&self) -> Span;

    fn join_span(&self, other: &impl Spanned) -> Span {
        let l = self.span();
        let r = other.span();
        usize::min(l.start, r.start)..usize::max(l.end, r.end)
    }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        self.as_ref().span()
    }
}
